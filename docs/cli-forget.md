```
restic forget -h

The "forget" command removes snapshots according to a policy. All snapshots are
first divided into groups according to "--group-by", and after that the policy
specified by the "--keep-*" options is applied to each group individually.
If there are not enough snapshots to keep one for each duration related
"--keep-{within-,}*" option, the oldest snapshot in the group is kept
additionally.

Please note that this command really only deletes the snapshot object in the
repository, which is a reference to data stored there. In order to remove the
unreferenced data after "forget" was run successfully, see the "prune" command.

Please also read the documentation for "forget" to learn about some important
security considerations.

EXIT STATUS
===========

Exit status is 0 if the command was successful.
Exit status is 1 if there was any error.
Exit status is 10 if the repository does not exist.
Exit status is 11 if the repository is already locked.
Exit status is 12 if the password is incorrect.

Usage:
  restic forget [flags] [snapshot ID] [...]

Flags:
  -l, --keep-last n                       keep the last n snapshots (use 'unlimited' to keep all snapshots)
  -H, --keep-hourly n                     keep the last n hourly snapshots (use 'unlimited' to keep all hourly snapshots)
  -d, --keep-daily n                      keep the last n daily snapshots (use 'unlimited' to keep all daily snapshots)
  -w, --keep-weekly n                     keep the last n weekly snapshots (use 'unlimited' to keep all weekly snapshots)
  -m, --keep-monthly n                    keep the last n monthly snapshots (use 'unlimited' to keep all monthly snapshots)
  -y, --keep-yearly n                     keep the last n yearly snapshots (use 'unlimited' to keep all yearly snapshots)
      --keep-within duration              keep snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-within-hourly duration       keep hourly snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-within-daily duration        keep daily snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-within-weekly duration       keep weekly snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-within-monthly duration      keep monthly snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-within-yearly duration       keep yearly snapshots that are newer than duration (eg. 1y5m7d2h) relative to the latest snapshot
      --keep-tag taglist                  keep snapshots with this taglist (can be specified multiple times) (default [])
      --unsafe-allow-remove-all           allow deleting all snapshots of a snapshot group
      --host host                         only consider snapshots for this host (can be specified multiple times) (default: $RESTIC_HOST)
      --tag tag[,tag,...]                 only consider snapshots including tag[,tag,...] (can be specified multiple times) (default [])
      --path path                         only consider snapshots including this (absolute) path (can be specified multiple times, snapshots must include all specified paths)
  -c, --compact                           use compact output format
  -g, --group-by group                    group snapshots by host, paths and/or tags, separated by comma (disable grouping with '') (default host,paths)
  -n, --dry-run                           do not delete anything, just print what would be done
      --prune                             automatically run the 'prune' command if snapshots have been removed
      --max-unused limit                  tolerate given limit of unused data (absolute value in bytes with suffixes k/K, m/M, g/G, t/T, a value in % or the word 'unlimited') (default "5%")
      --max-repack-size size              stop after repacking this much data in total (allowed suffixes for size: k/K, m/M, g/G, t/T)
      --repack-cacheable-only             only repack packs which are cacheable
      --repack-small                      repack pack files below 80% of target pack size
      --repack-uncompressed               repack all uncompressed data
      --repack-smaller-than below-limit   pack below-limit packfiles (allowed suffixes: k/K, m/M)
  -h, --help                              help for forget

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
