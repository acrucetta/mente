
## Tech Independence Reference

### SSH into Server

`ssh andres@andrescn.me`

### Sync Files with Server

MacOS Only:

`rsync -avz Folder andres@andrescn.me:/mnt/`

Sync with cron:

1. `crontab -l` - list of files
2. `cron xxx rsync...` - sync on a given time period

### Use Storage

1. Type find /mnt
2. You should see a long list of the files you uploaded.
3. Type m-x to detach your encrypted storage.
4. Type find /mnt again, and now you should see nothing there! Congratulations! You now see how this will work in the future:
5. Log in and type “m” to attach your encrypted storage.
6. Upload your files with rsync or FreeFileSync.
7. Log in and type “m-x” to detach the storage, for security.

