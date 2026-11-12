"""Fail-closed inventory coverage and bounded exact-filter invocation checks."""
import json
from pathlib import Path
import subprocess
import unittest
from unittest.mock import patch

from run_ci_store_shard import ROTATIONS_PREFIX, commands, partition, run


class StoreShardTests(unittest.TestCase):
    def test_disjoint_complete_partition(self):
        names = ["other::test", "ledger::test", "ledger::continuation::test",
                 "ledger::continuation_other::test", ROTATIONS_PREFIX + "test"]
        groups = partition("\n".join(f"{name}: test" for name in names))
        self.assertEqual(groups["journal"], [names[1], names[3]])
        self.assertEqual(groups["continuation"], [names[2]])
        self.assertEqual(groups["rotations"], [names[4]])
        self.assertEqual(sorted(sum(groups.values(), [])), sorted(names))

    def test_invalid_inventory_refused(self):
        valid = ("a: test\nledger::a: test\nledger::continuation::a: test\n"
                 + ROTATIONS_PREFIX + "a: test")
        for listing in ("", "a: test", valid.rsplit("\n", 1)[0], valid + "\na: test",
                        valid + "\nunknown", valid + "\nx: benchmark"):
            with self.subTest(listing=listing), self.assertRaises(ValueError):
                partition(listing)

    def test_batches_preserve_exact_names_and_bounds(self):
        names = ["test_" + str(i) + "x" * 400 for i in range(200)]
        batches = list(commands("test.exe", names))
        self.assertEqual([n for batch in batches for n in batch[3:]], names)
        for batch in batches:
            self.assertEqual(batch[:3], ["test.exe", "--exact", "--test-threads=1"])
            self.assertLessEqual(len(batch) - 3, 64)
            self.assertLess(len(" ".join(batch)), 24000)

    def test_empty_duplicate_and_oversized_selections_refused(self):
        for names in ([], ["a", "a"], ["x" * 4097]):
            with self.subTest(names=names), self.assertRaises(ValueError):
                list(commands("test.exe", names))

    def test_real_runner_propagates_test_failure(self):
        artifact = json.dumps({"reason": "compiler-artifact",
                               "target": {"name": "tig_store"},
                               "profile": {"test": True},
                               "executable": str(Path(__file__).resolve())})
        build = subprocess.CompletedProcess([], 0, artifact, "")
        listing = subprocess.CompletedProcess([], 0,
            "a: test\nledger::b: test\nledger::continuation::c: test\n"
            + ROTATIONS_PREFIX + "d: test", "")
        failed = subprocess.CalledProcessError(1, ["test"])
        with patch("run_ci_store_shard.subprocess.run",
                   side_effect=[build, listing, failed]) as execute:
            with self.assertRaises(subprocess.CalledProcessError):
                run("continuation")
        self.assertEqual(execute.call_args_list[-1].args[0][1:],
                         ["--exact", "--test-threads=1", "ledger::continuation::c"])

    def test_build_failure_and_missing_artifact_refused(self):
        for result, error in ((subprocess.CompletedProcess([], 1, "", ""),
                               subprocess.CalledProcessError),
                              (subprocess.CompletedProcess([], 0, "", ""),
                               ValueError)):
            with patch("run_ci_store_shard.subprocess.run", return_value=result):
                with self.assertRaises(error):
                    run("store")


if __name__ == "__main__":
    unittest.main()
