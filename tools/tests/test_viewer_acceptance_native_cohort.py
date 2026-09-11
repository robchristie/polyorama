"""Authored/mock checks only: never open a display or launch an application."""
import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch

SPEC = importlib.util.spec_from_file_location('cohort', Path(__file__).resolve().parents[1] / 'viewer-acceptance-native-cohort.py')
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)


class CohortTests(unittest.TestCase):
    def test_five_slots_survive_each_failed_start_without_retries(self):
        calls = []

        def run(index):
            calls.append(index)
            if index in (1, 3, 5):
                raise OSError('authored failed app start')
            return dict(index=index, harness_exit=4, failures=['authored incomplete cycles'])

        rows = C.five_slots(run)
        self.assertEqual(calls, [1, 2, 3, 4, 5])
        self.assertEqual([r['index'] for r in rows], calls)
        self.assertTrue(all(r['failures'] for r in rows))

    def test_grant_requires_committed_identity_and_stopped_owner(self):
        grant = dict(schema='viewer_native_cohort_grant/1', protocol_commit='a' * 40,
                     output_name='viewer-acceptance-native-cohort-authored', display=':177',
                     port=8194, execute=True, authored_probe_owner_stopped=True,
                     granted_by='authored coordinator', granted_utc='authored timestamp')
        args = ('a' * 40, grant['output_name'], ':177', 8194)
        C.validate_grant(grant, *args)
        for key in grant:
            with self.subTest(key=key):
                changed = {k: v for k, v in grant.items() if k != key}
                with self.assertRaises(ValueError):
                    C.validate_grant(changed, *args)

    def test_committed_gate_rejects_uncommitted_runner(self):
        with patch.object(C, 'git', side_effect=[b'abc\n', b'committed']), \
                patch.object(C, 'read', return_value=b'changed'):
            with self.assertRaisesRegex(ValueError, 'uncommitted protocol'):
                C.committed('abc')

    def test_contract_preserves_every_inherited_number_and_cycles(self):
        protocol = C.load(C.PROTOCOL)
        C.validate_contract(protocol)
        for key in protocol['budget']:
            changed = copy.deepcopy(protocol)
            changed['budget'][key] += 1
            with self.assertRaisesRegex(ValueError, 'budget drift'):
                C.validate_contract(changed)
        changed = copy.deepcopy(protocol)
        changed['captures']['offset_seconds'] = [1, 5]
        with self.assertRaisesRegex(ValueError, 'capture schedule'):
            C.validate_contract(changed)

    def test_native_arguments_bind_existing_harness_and_fixed_workload(self):
        protocol = C.load(C.PROTOCOL)
        for index in range(1, 6):
            args = C.journey_arguments(protocol, Path('/authored/native'), 'http://127.0.0.1:8194', index)
            self.assertEqual(args[args.index('--mode') + 1], 'native')
            self.assertEqual(args[args.index('--workload') + 1], protocol['workload']['path'])
            self.assertEqual(args[args.index('--thresholds') + 1], protocol['thresholds']['path'])
            self.assertIn('--memory-diagnostics', args)
            self.assertNotIn('--authored-immediate-completion', args)
            self.assertEqual(args[args.index('--server-cache-state') + 1],
                             'uncontrolled-first-observation' if index == 1 else 'warm-server')

    def test_observer_launches_actual_binary_once_and_reports_clock_interval(self):
        connection = Mock()
        observer = C.LaunchObserver(connection, '/authored/native')
        child = Mock(pid=42)
        with patch.object(C.subprocess, 'Popen', return_value=child) as popen, \
                patch.object(C, 'process_identity', return_value=dict(pid=42, start_time_ticks=7, process_group=42)), \
                patch.object(C.time, 'monotonic_ns', side_effect=[100, 110]), \
                patch.object(C.time, 'time_ns', return_value=1000):
            self.assertIs(observer.Popen(['/authored/native'], start_new_session=True), child)
            with self.assertRaisesRegex(ValueError, 'one native launch'):
                observer.Popen(['/authored/native'])
            popen.assert_called_once_with(['/authored/native'], start_new_session=True)
        record = connection.send.call_args.args[0]
        self.assertEqual((record['launch_monotonic_ns'], record['popen_return_monotonic_ns']), (100, 110))

    def test_capture_targets_owned_root_with_hard_bounds(self):
        command = C.capture_command('/authored/import', ':177', Path('/approved/run/capture.png'))
        self.assertEqual(command[:5], ['/authored/import', '-display', ':177', '-window', 'root'])
        self.assertIn('PNG24:/approved/run/capture.png', command)
        self.assertEqual(command[command.index('-snaps') + 1], '1')
        with patch.object(C.resource, 'setrlimit') as limit:
            C.capture_limits()
        self.assertIn(((C.resource.RLIMIT_FSIZE, (8 << 20, 8 << 20)),),
                      [(call.args,) for call in limit.call_args_list])

    def test_exited_native_does_not_capture_stale_display_or_retry(self):
        with patch.object(C, 'alive', return_value=False), patch.object(C.subprocess, 'Popen') as launch:
            for offset in C.OFFSETS:
                row, process = C.begin_capture({}, Path('/authored'), ':177',
                    dict(identity={'pid': 42}, launch_monotonic_ns=0), offset)
                self.assertIsNone(process)
                self.assertIn('no retry', row['failure'])
            launch.assert_not_called()

    def test_capture_timeout_retains_partial_image_and_does_not_retry(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'partial.png'
            path.write_bytes(b'authored partial')
            row = dict(attempted_monotonic_ns=0, capture_identity={'pid': 42}, image=str(path))
            child = Mock()
            child.poll.return_value = None
            child.wait.return_value = -9
            with patch.object(C.time, 'monotonic_ns', return_value=2_000_000_000), \
                    patch.object(C, 'kill_owned') as kill:
                self.assertTrue(C.finish_capture(row, child))
                kill.assert_called_once_with({'pid': 42})
            self.assertEqual(path.read_bytes(), b'authored partial')
            self.assertEqual(row['exit_code'], -9)
            self.assertIn('deadline', row['failure'])

    def test_cleanup_refuses_pid_reuse_or_unowned_group(self):
        record = dict(pid=42, start_time_ticks=7, process_group=42)
        with patch.object(C, 'process_identity', return_value=dict(record, start_time_ticks=8)), \
                patch.object(C, 'read', return_value=b'42 (authored) S 1 42'), \
                patch.object(C.os, 'killpg') as kill:
            C.kill_owned(record)
            kill.assert_not_called()
        with patch.object(C, 'alive', return_value=True), patch.object(C.os, 'killpg') as kill:
            with self.assertRaisesRegex(ValueError, 'unowned group'):
                C.kill_owned(dict(record, process_group=1))
            kill.assert_not_called()

    def test_zombie_native_is_unavailable_for_capture(self):
        with patch.object(C, 'read', return_value=b'42 (authored) Z 1 42'), \
                patch.object(C, 'process_identity') as identity:
            self.assertFalse(C.alive({'pid': 42}))
            identity.assert_not_called()

    def test_outer_deadline_kills_native_at_660_without_extra_launch(self):
        with tempfile.TemporaryDirectory() as temporary:
            now, living, kills = [0], [True], []
            native = dict(pid=42, start_time_ticks=7, process_group=42)
            launch = dict(identity=native, launch_monotonic_ns=0)
            receiver, sender, worker, context = Mock(), Mock(), Mock(), Mock()
            receiver.poll.return_value = True
            receiver.recv.return_value = launch
            worker.is_alive.side_effect = lambda: now[0] < 665 * 10**9
            worker.exitcode = 4
            context.Pipe.return_value = (receiver, sender)
            context.Process.return_value = worker

            def advance(_seconds):
                now[0] += 10**9

            def kill(_identity, *args):
                if living[0]:
                    kills.append(now[0])
                    living[0] = False

            def capture(_protocol, _path, _display, _launch, offset):
                return dict(offset_seconds=offset, failure='authored capture unavailable'), None

            with patch.object(C.multiprocessing, 'get_context', return_value=context), \
                    patch.object(C.time, 'monotonic_ns', side_effect=lambda: now[0]), \
                    patch.object(C.time, 'sleep', side_effect=advance), \
                    patch.object(C, 'alive', side_effect=lambda _: living[0]), \
                    patch.object(C, 'kill_owned', side_effect=kill), \
                    patch.object(C, 'begin_capture', side_effect=capture) as captures, \
                    patch.object(C, 'mapped_files', return_value={'unavailable': 'authored'}):
                result = C.one_run(C.load(C.PROTOCOL), Path(temporary), 1, ':177', 'http://authored')
            self.assertEqual(kills, [660 * 10**9])
            self.assertEqual([c.args[-1] for c in captures.call_args_list], [2, 5])
            worker.start.assert_called_once()
            self.assertEqual(result['harness_exit'], 4)
            self.assertIn('660-second', result['failures'][0])
    def test_no_missing_trace_is_manufactured_for_benchmark(self):
        with tempfile.TemporaryDirectory() as temporary, patch.object(C.subprocess, 'run') as run:
            group = Path(temporary)
            report = C.assess({'benchmark': {'path': '/authored/benchmark'}}, group, 1)
            run.assert_not_called()
            self.assertIn('no synthetic', report['benchmark']['unavailable'])
            self.assertIsNone(report['current_growth'])
            self.assertIsNone(report['rss']['process_peak_rss_bytes']['within_ceiling'])

    def test_growth_requires_all_ten_bracketed_cycles_and_keeps_rss_gate(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            samples, attachments = [], []
            for i in range(10):
                for j in range(2):
                    samples.append(dict(identity={'value': {'pid': 42, 'start_time_ticks': 7}},
                        status={'value': {'VmRSS': {'value': 1000 + i * 10 + j}, 'VmHWM': {'value': 999999}}},
                        read_started_monotonic_ns=i * 10 + j, read_finished_monotonic_ns=i * 10 + j,
                        smaps_rollup={'value': None}, mapping_categories={'value': None}))
                attachments.append(dict(marker=dict(kind='cycle-revisited', phase_label=f'cycle-{i+1:02}',
                    pid=42, start_time_ticks=7), before_sample={'value': 2*i}, after_sample={'value': 2*i+1}))
            diagnostic = dict(samples=samples, phase_attachments=attachments)
            (root / 'process-memory-diagnostics.json').write_text(json.dumps(diagnostic))
            (root / 'composed-trace.json').write_text(json.dumps(dict(observations={
                'process_peak_rss_bytes': {'value': 278794240},
                'process_group_sampled_peak_rss_bytes': {'value': 250000000},
                'observed_process_high_water_sum_bytes': {'value': 280000000}})))
            report = C.summarise(root)
            self.assertEqual(report['current_growth']['last_minus_first_bytes'], 90)
            self.assertEqual(len(report['cycle_brackets']), 10)
            self.assertTrue(all(v['within_ceiling'] is False for v in report['rss'].values()))
            self.assertIn('never add or subtract', report['boundary'])
            attachments[4]['before_sample'] = dict(value=None, unavailable_reason='authored gap')
            (root / 'process-memory-diagnostics.json').write_text(json.dumps(diagnostic))
            self.assertIsNone(C.summarise(root)['current_growth'])

    def test_failed_trace_export_still_reports_actual_sampler_rss(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / 'process-memory.json').write_text(json.dumps(dict(
                samples=[{'rss_bytes': 250000000}, {'rss_bytes': 210000000}],
                observed_pid_high_water_bytes={'42': 278794240})))
            (root / 'app.json').write_text(json.dumps(dict(process_peak_rss_bytes=278794240)))
            result = C.summarise(root)
            self.assertEqual(result['rss']['process_group_sampled_peak_rss_bytes']['bytes'], 250000000)
            self.assertEqual(result['rss']['observed_process_high_water_sum_bytes']['bytes'], 278794240)
            self.assertTrue(all(g['within_ceiling'] is False for g in result['rss'].values()))
            self.assertFalse((root / 'composed-trace.json').exists())


if __name__ == '__main__':
    unittest.main()
