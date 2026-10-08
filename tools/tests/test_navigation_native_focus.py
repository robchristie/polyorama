"""Check the production traversal against delayed and failed observations."""

import unittest

from tools.navigation_native_focus import focus_navigation_target


class Clock:
    def __init__(self):
        self.now = 0.0

    def monotonic(self):
        return self.now

    def sleep(self, seconds):
        self.now += seconds


def state(frame, focused):
    return {"frame": frame, "navigation_fixture": {"selected": "home", "activations": 0},
            "ui_snapshot": {"nodes": [{"id": focused, "focused": True}]}}


class NavigationNativeFocusTests(unittest.TestCase):
    def test_delayed_presentation_blocks_another_tab(self):
        clock, transitions, presses = Clock(), [], []
        current, pending = state(0, "initial"), None

        def snapshot():
            nonlocal current, pending
            if pending and clock.now >= pending[0]:
                current, pending = pending[1], None
            return current

        def press():
            nonlocal pending
            self.assertIsNone(pending, "another key preceded the prior presentation")
            presses.append(clock.now)
            pending = (clock.now + 0.2, state(len(presses), "tasks" if len(presses) == 2 else "home"))

        result = focus_navigation_target(snapshot, press, "tasks", transitions,
            monotonic=clock.monotonic, sleep=clock.sleep)
        self.assertEqual(result["frame"], 2)
        self.assertGreaterEqual(presses[1] - presses[0], 0.2)
        self.assertEqual([t["after"]["frame"] for t in transitions], [1, 2])

    def test_fixed_delay_discriminator_can_outpace_presentation(self):
        clock, presses = Clock(), []
        due = None

        def press():
            nonlocal due
            presses.append(due is not None and clock.now < due)
            due = clock.now + 0.2

        press()
        clock.sleep(0.06)
        press()
        self.assertEqual(presses, [False, True])

    def test_stale_frame_times_out_without_another_key(self):
        clock, presses, transitions = Clock(), [], []
        with self.assertRaises(TimeoutError):
            focus_navigation_target(lambda: state(1, "home"), lambda: presses.append(1), "tasks", transitions,
                timeout_seconds=0.2, monotonic=clock.monotonic, sleep=clock.sleep)
        self.assertEqual(presses, [1])
        self.assertEqual(transitions[-1]["after"]["frame"], 1)
        self.assertEqual(transitions[-1]["error"], "presentation did not advance")

    def test_unreachable_stops_at_forty_actions(self):
        current, transitions, presses = [state(0, "home")], [], []

        def press():
            presses.append(1)
            current[0] = state(len(presses), "home")

        with self.assertRaisesRegex(AssertionError, "after 40 actions"):
            focus_navigation_target(lambda: current[0], press, "tasks", transitions)
        self.assertEqual(len(presses), 40)
        self.assertEqual(transitions[-1]["after"]["frame"], 40)

    def test_target_on_final_permitted_action_is_accepted(self):
        current, transitions, presses = [state(0, "home")], [], []

        def press():
            presses.append(1)
            current[0] = state(len(presses), "tasks" if len(presses) == 40 else "home")

        result = focus_navigation_target(lambda: current[0], press, "tasks", transitions)
        self.assertEqual(result["frame"], 40)
        self.assertEqual(len(presses), 40)

    def test_failed_dispatch_propagates_without_another_key(self):
        transitions, presses = [], []
        error = RuntimeError("xdotool failed")

        def press():
            presses.append(1)
            raise error

        with self.assertRaises(RuntimeError) as result:
            focus_navigation_target(lambda: state(1, "home"), press, "tasks", transitions)
        self.assertIs(result.exception, error)
        self.assertEqual(presses, [1])
        self.assertEqual(transitions[0]["dispatch"], "failed")

    def test_tab_cannot_select_or_activate_a_destination(self):
        current, transitions = [state(0, "home")], []

        def press():
            current[0] = state(1, "tasks")
            current[0]["navigation_fixture"]["selected"] = "tasks"

        with self.assertRaisesRegex(AssertionError, "changed selection or activation"):
            focus_navigation_target(lambda: current[0], press, "tasks", transitions)
