# Good Pracices

This is a file that will serve as a resource for good practices on multiple project organization. Please refer to this if you have some doubts about how you should proceed.

## Git
---
Please note that for any new feature you are working on, it should start from the top of develop branch and be worked separatly. When you're finished with your work, open a pull request and add one maintainer to review your code. When you get your approvals, fix the conflicts (if needed), clean your history (if needed), and merge the PR.

### What should I do if I cannot push my commits to the remote branch?
If this happens, then your local branch diverged from the remote one. We care about our git history, thus we should not make unnecessary merges, thus we need to merge a branch without an explicit commit. How we do that, you ask? Easy, we rebase the branch.
To do that, just pull-rebase before pushing your local changes with:

```
git pull --rebase
```

This way your local changes will be put on the top of the remote branch and no unnecessary merges will appear in history.

### I opened a Pull Request and made unncessary commits just for the sake of review simplicity. How do I clean my history?
Rebase is the answer, however instead of doing it against the remote branch, you do it locally, against your commits. Just execute:
```
git rebase -i <the commit before the one you want to change>
```

Please understand that this comman (on mac and linux) will open vim, and you should have some notions of that text editor.

