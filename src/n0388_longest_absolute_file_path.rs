/**
 * [0388] Longest Absolute File Path
 *
 * Suppose we abstract our file system by a string in the following manner:
 *
 * The string "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" represents:
 *
 * dir
 *     subdir1
 *     subdir2
 *         file.ext
 *
 *
 * The directory dir contains an empty sub-directory subdir1 and a sub-directory subdir2 containing a file file.ext.
 *
 * The string "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext" represents:
 *
 * dir
 *     subdir1
 *         file1.ext
 *         subsubdir1
 *     subdir2
 *         subsubdir2
 *             file2.ext
 *
 *
 * The directory dir contains two sub-directories subdir1 and subdir2. subdir1 contains a file file1.ext and an empty second-level sub-directory subsubdir1. subdir2 contains a second-level sub-directory subsubdir2 containing a file file2.ext.
 *
 * We are interested in finding the longest (number of characters) absolute path to a file within our file system. For example, in the second example above, the longest absolute path is "dir/subdir2/subsubdir2/file2.ext", and its length is 32 (not including the double quotes).
 *
 * Given a string representing the file system in the above format, return the length of the longest absolute path to file in the abstracted file system. If there is no file in the system, return 0.
 *
 * Note:
 *
 * The name of a file contains at least a . and an extension.
 * The name of a directory or sub-directory will not contain a ..
 *
 *
 *
 * Time complexity required: O(n) where n is the size of the input string.
 *
 * Notice that a/aa/aaa/file1.txt is not the longest file path, if there is another path aaaaaaaaaaaaaaaaaaaaa/sth.png.
 */
pub struct Solution {}

// submission codes start here

fn has_children(index: usize, tree: &[&str], depth: usize) -> bool {
    index + 1 < tree.len()
        && tree[index + 1].len() - tree[index + 1].trim_start_matches('\t').len() == depth
}

fn longest_path_child(parent: &str, index: usize, tree: &[&str], depth: usize, length: i32) -> i32 {
    let parent_len: i32 = length + (parent.len() as i32) + 1;
    if has_children(index, tree, depth) {
        let mut max_len: i32 = 0;
        for i in (index + 1)..tree.len() {
            let mut child = tree[i];
            let gap = child.len() - child.trim_start_matches('\t').len();
            if gap == depth {
                child = child.trim_start_matches('\t');
                let child_len = longest_path_child(child, i, tree, depth + 1, parent_len);
                max_len = i32::max(max_len, child_len);
            } else if gap > depth {
                continue;
            } else {
                break;
            }
        }
        max_len
    } else if parent.contains('.') {
        parent_len - 1
    } else {
        0
    }
}

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let tree: Vec<&str> = input.split('\n').collect();
        let mut max_len: i32 = 0;
        for (i, root) in tree
            .iter()
            .enumerate()
            .filter(|(_, root)| root.trim_start_matches('\t').len() == root.len())
        {
            max_len = i32::max(longest_path_child(root, i, &tree, 1, 0), max_len);
        }
        max_len
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0388() {
        assert_eq!(
            20,
            Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string())
        );
        assert_eq!(32, Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()));
    }
}
