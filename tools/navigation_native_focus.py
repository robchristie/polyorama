"""Observe each native navigation presentation before another physical Tab."""

import time


def _observation(state):
    fixture = state["navigation_fixture"]
    return {
        "frame": state["frame"],
        "focused_ids": [node["id"] for node in state["ui_snapshot"]["nodes"] if node["focused"]],
        "selected": fixture["selected"],
        "activations": fixture["activations"],
    }


def focus_navigation_target(
    snapshot, press_tab, target_id, transitions, *, max_actions=40,
    timeout_seconds=10, monotonic=time.monotonic, sleep=time.sleep,
):
    """Keep the action bound and stop when a dispatch or presentation fails.

    A newer frame is an observation barrier. It does not acknowledge GPU work
    or prove that a particular native input event was consumed.
    """
    deadline = monotonic() + timeout_seconds
    state = snapshot()
    if target_id in _observation(state)["focused_ids"]:
        return state
    for action in range(1, max_actions + 1):
        before = _observation(state)
        transition = {"action": action, "target_id": target_id, "before": before,
                      "after": None, "dispatch": "pending"}
        transitions.append(transition)
        try:
            press_tab()
        except BaseException as error:
            transition.update(dispatch="failed", error=str(error))
            raise
        transition["dispatch"] = "acknowledged by xdotool"
        while True:
            state = snapshot()
            after = _observation(state)
            transition["after"] = after
            if after["frame"] > before["frame"]:
                break
            if monotonic() >= deadline:
                transition["error"] = "presentation did not advance"
                raise TimeoutError(f"native navigation presentation did not advance after Tab {action}")
            sleep(0.03)
        if (after["selected"], after["activations"]) != (before["selected"], before["activations"]):
            raise AssertionError("native navigation Tab changed selection or activation")
        # Examine every after-state, including the final permitted action.
        if target_id in after["focused_ids"]:
            return state
        if monotonic() >= deadline:
            raise TimeoutError("native navigation focus traversal deadline exceeded")
    raise AssertionError(f"keyboard did not reach {target_id} after {max_actions} actions")
