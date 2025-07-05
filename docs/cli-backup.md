```
restic backup -h

The "backup" command creates a new snapshot and saves the files and directories
given as the arguments.

EXIT STATUS
===========

Exit status is 0 if the command was successful.
Exit status is 1 if there was a fatal error (no snapshot created).
Exit status is 3 if some source data could not be read (incomplete snapshot created).
Exit status is 10 if the repository does not exist.
Exit status is 11 if the repository is already locked.
Exit status is 12 if the password is incorrect.

Usage:
  restic backup [flags] [FILE/DIR] ...

Flags:
  -n, --dry-run                                do not upload or write any data, just show what would be done
  -e, --exclude pattern                        exclude a pattern (can be specified multiple times)
      --exclude-caches                         excludes cache directories that are marked with a CACHEDIR.TAG file. See https://bford.info/cachedir/ for the Cache Directory Tagging Standard
      --exclude-cloud-files                    excludes online-only cloud files (such as OneDrive Files On-Demand)
      --exclude-file file                      read exclude patterns from a file (can be specified multiple times)
      --exclude-if-present filename[:header]   takes filename[:header], exclude contents of directories containing filename (except filename itself) if header of that file is as provided (can be specified multiple times)
      --exclude-larger-than size               max size of the files to be backed up (allowed suffixes: k/K, m/M, g/G, t/T)
      --files-from file                        read the files to backup from file (can be combined with file args; can be specified multiple times)
      --files-from-raw file                    read the files to backup from file (can be combined with file args; can be specified multiple times)
      --files-from-verbatim file               read the files to backup from file (can be combined with file args; can be specified multiple times)
  -f, --force                                  force re-reading the source files/directories (overrides the "parent" flag)
  -g, --group-by group                         group snapshots by host, paths and/or tags, separated by comma (disable grouping with '') (default host,paths)
  -h, --help                                   help for backup
  -H, --host hostname                          set the hostname for the snapshot manually (default: $RESTIC_HOST). To prevent an expensive rescan use the "parent" flag
      --iexclude pattern                       same as --exclude pattern but ignores the casing of filenames
      --iexclude-file file                     same as --exclude-file but ignores casing of filenames in patterns
      --ignore-ctime                           ignore ctime changes when checking for modified files
      --ignore-inode                           ignore inode number and ctime changes when checking for modified files
      --no-scan                                do not run scanner to estimate size of backup
  -x, --one-file-system                        exclude other file systems, don't cross filesystem boundaries and subvolumes
      --parent snapshot                        use this parent snapshot (default: latest snapshot in the group determined by --group-by and not newer than the timestamp determined by --time)
      --read-concurrency n                     read n files concurrently (default: $RESTIC_READ_CONCURRENCY or 2)
      --skip-if-unchanged                      skip snapshot creation if identical to parent snapshot
      --stdin                                  read backup from stdin
      --stdin-filename filename                filename to use when reading from stdin (default "stdin")
      --stdin-from-command                     interpret arguments as command to execute and store its stdout
      --tag tags                               add tags for the new snapshot in the format `tag[,tag,...]` (can be specified multiple times) (default [])
      --time time                              time of the backup (ex. '2012-11-01 22:08:41') (default: now)
      --use-fs-snapshot                        use filesystem snapshot where possible (currently only Windows VSS)
      --with-atime                             store the atime for all files and directories

Global Flags:
      --cacert file                      file to load root certificates from (default: use system certificates or $RESTIC_CACERT)
      --cache-dir directory              set the cache directory. (default: use system default cache directory)
      --cleanup-cache                    auto remove old cache directories
      --compression mode                 compression mode (only available for repository format version 2), one of (auto|off|max) (default: $RESTIC_COMPRESSION) (default auto)
      --http-user-agent string           set a http user agent for outgoing http requests
      --insecure-no-password             use an empty password for the repository, must be passed to every restic command (insecure)
      --insecure-tls                     skip TLS certificate verification when connecting to the repository (insecure)
      --json                             set output mode to JSON for commands that support it
      --key-hint key                     key ID of key to try decrypting first (default: $RESTIC_KEY_HINT)
      --limit-download rate              limits downloads to a maximum rate in KiB/s. (default: unlimited)
      --limit-upload rate                limits uploads to a maximum rate in KiB/s. (default: unlimited)
      --no-cache                         do not use a local cache
      --no-extra-verify                  skip additional verification of data before upload (see documentation)
      --no-lock                          do not lock the repository, this allows some operations on read-only repositories
  -o, --option key=value                 set extended option (key=value, can be specified multiple times)
      --pack-size size                   set target pack size in MiB, created pack files may be larger (default: $RESTIC_PACK_SIZE)
      --password-command command         shell command to obtain the repository password from (default: $RESTIC_PASSWORD_COMMAND)
  -p, --password-file file               file to read the repository password from (default: $RESTIC_PASSWORD_FILE)
  -q, --quiet                            do not output comprehensive progress report
  -r, --repo repository                  repository to backup to or restore from (default: $RESTIC_REPOSITORY)
      --repository-file file             file to read the repository location from (default: $RESTIC_REPOSITORY_FILE)
      --retry-lock duration              retry to lock the repository if it is already locked, takes a value like 5m or 2h (default: no retries)
      --stuck-request-timeout duration   duration after which to retry stuck requests (default 5m0s)
      --tls-client-cert file             path to a file containing PEM encoded TLS client certificate and private key (default: $RESTIC_TLS_CLIENT_CERT)
  -v, --verbose                          be verbose (specify multiple times or a level using --verbose=n, max level/times is 2
```
