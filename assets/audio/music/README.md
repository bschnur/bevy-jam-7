### Hi Martin!

Place your music files here.

I request the following:

- 48kHz sample rate
- normalized to -6dB
- no hard clicks (protect the ears of headphone users)
- format: .ogg preferred, .wav acceptable

### git

Ostensibly you retrieved this repo the first time a la:

```bash
git clone git@github.com:bschnur/bevy-jam-7.git [optional-directory-name]
```

To get the latest changes, you can run:

```bash
git pull
```

To check the status of your local working directory vis-a-vis the repo:

```bash
git status
```

and to stage files:

```bash
git add [filename]
```

This needs to be done to start tracking newly created files, but also for modified files to incorporate your changes in the commit.

Once staged, files can be added to a commit with:

```bash
git commit
```

This will open a text editor with some comments - ignore those and type your commit message (e.g., "added main theme" without the quotes). Save and close the file to finalize the commit.

Alternatively, close without saving / without any uncommented text to abort the commit.

Once you have created a commit in your local workspace, you can push it to the Github repo with:

```bash
git push
```

That should cover the basics. Any edge cases or general questions - I'm happy to help.