
# r(ust)sync

## Overview
r(ust)sync is a synchronization tool written in Rust that ensures real-time data consistency between two or more locations. It continuously monitors changes and keeps locations synchronized by mirroring file operations such as creation, deletion, and modification.
## How it works
- **Real-Time Synchronization**:
  - Automatically syncs file changes, ensuring all specified locations remain up to date.
- **Initial Synchronization**:
  - Detects and recognize differences between locations at startup:
    - Files missing in one location are copied from the other.
    - For conflicting files, the most recent version is retained.
- **Change Detection**:
  - Monitors locations for file operations and applies them consistently:
    - **Create**: New files in one location are replicated in the other(s).
    - **Delete**: Deleted files are removed from all locations.
    - **Modify**: Updates to files are propagated across locations.
- **Read-Only Support for ZIP**: ZIP archives are treated as read-only sources for synchronization.

## Supported Location Formats
A location is specified in the following format:
```plaintext
<LOCATION_TYPE>:<Path_in_location>
```
Examples:
- `ftp:user:password@URL/a.b.c`
- `zip:C:/abc/d.zip`
- `folder:C:/aaa`

### Notes:
- **FTP**: Requires credentials in the format `user:password`.
- **ZIP**: Treated as a read-only source; changes cannot be applied to ZIP archives.
- **Folders**: Local directories are fully synchronized.

## Applied Technologies
- **Language**: Rust
- **Libraries/Crates**:
  - `ftp` for FTP communication.
  - `zip` for handling ZIP archives.
  - `notify` for filesystem event monitoring.
- **Concurrency**: Utilizes multithreading to handle multiple locations efficiently.

## Objectives
- Maintain data consistency with minimal manual intervention.
- Prioritize reliability and performance in handling large datasets.
- Provide clear, real-time feedback on synchronization status.

## Usage Instructions
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your_username/advanced_rsync.git
   cd advanced_rsync
   ```

2. **Build the Application**:
   Ensure you have Rust installed. Build the project with:
   ```bash
   cargo build --release
   cargo install --path .
   ```
   3. **Modify config file**:
   ```bash
   adv_rsync --set <LOCATION_TYPE>:<Path_in_location> ..
   ```
   Or you can simply modify the config file located in:
    ```bash
   ~/.adv_rsync/cfg/locations.cfg
   ```

4. **Run the Application**:
   ```bash
   adv_rsync
   ```


## Potential Improvements (future updates)
- Add support for advanced conflict resolution strategies (Delta).
- Enable optional encryption for sensitive files during transfer.
- Implement rate-limiting for FTP synchronization.
- Add SFTP functionality.


## License
This project is licensed under the GNU License. See the `LICENSE` file for details.

