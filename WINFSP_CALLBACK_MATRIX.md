# WinFsp Read-Only Callback Matrix

| Callback | Decision | Note |
|---|---|---|
| `Init` | `allow` | initialise read-only filesystem instance |
| `GetVolumeInfo` | `allow` | report read-only volume metadata |
| `GetSecurityByName` | `allow` | conservative read-only security metadata |
| `Create` | `refuse` | no file creation in read-only MVP |
| `Open` | `allow_readonly` | open existing files/directories read-only |
| `Read` | `allow` | read file data through apfs-vfs |
| `Write` | `refuse` | all writes refused |
| `Flush` | `allow_noop` | no APFS media mutation |
| `GetFileInfo` | `allow` | stat/getattr mapping |
| `SetBasicInfo` | `refuse` | metadata mutation refused |
| `SetFileSize` | `refuse` | truncate/extend refused |
| `CanDelete` | `refuse` | deletion refused |
| `Rename` | `refuse` | rename refused |
| `ReadDirectory` | `allow` | directory listing through apfs-vfs |
| `GetReparsePoint` | `allow_if_symlink` | symlink metadata only where supported |
| `SetReparsePoint` | `refuse` | metadata mutation refused |
| `DeleteReparsePoint` | `refuse` | metadata mutation refused |

This is a contract for the future live adapter. It is not a live mount implementation yet.
