/** Observe the existing browser traversal, including its final allowed action. */
export async function focusNavigationDestination(destination, { target, pressTab, waitForTab, wait }) {
  for (let attempt = 0; attempt < 45; attempt++) {
    const observation = await target(destination);
    if (observation.node.focused) return;
    await pressTab();
    // A fixed delay can leave the Tab queued behind a slow application frame.
    // Do not send another key until this traversal has reached egui.
    await waitForTab(observation.tab_input_epoch);
    await wait(60);
  }
  // Earlier pre-action reads observe the preceding Tab; the last Tab needs one too.
  if ((await target(destination)).node.focused) return;
  throw new Error(`Tab did not reach ${destination}`);
}
