"""Path boundary fixtures for the explicit project orchestrator."""
import tempfile
import unittest
from pathlib import Path
from restore_project import parents, relative


class Paths(unittest.TestCase):
    def test_portable_paths(self):
        self.assertEqual(relative(".local/tig-cache/objects/sha256/aa/aabb"), ".local/tig-cache/objects/sha256/aa/aabb")
        for path in ("../outside", "/absolute", "C:/drive", "a\\b", "a//b", "a/./b", ".git/config", ".GiT/config", "a/NUL.txt", "a/trailing. "):
            with self.subTest(path=path), self.assertRaises(ValueError):
                relative(path)

    def test_preview_never_creates_parents(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.assertEqual(parents(root, "data/raw/input", False), root / "data/raw/input")
            self.assertFalse((root / "data").exists())
            parents(root, "data/raw/input", True)
            self.assertTrue((root / "data/raw").is_dir())

    def test_file_parent_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "data").write_text("keep")
            with self.assertRaises(ValueError):
                parents(root, "data/raw/input", False)
            self.assertEqual((root / "data").read_text(), "keep")


if __name__ == "__main__":
    unittest.main()
