*Redundant Array of Independent Disks*

**Key Terms**
- **Parity**: Parity is a method used in some RAID levels (like RAID 5 and 6) to provide fault tolerance. It's a calculated value that can be used to reconstruct data if a drive fails.
- **Stripe Size**: Stripe size refers to the amount of data written to each disk in a RAID array before moving to the next disk.

RAID is a storage technology that combines multiple disk drive components into a logical unit for data redundancy and performance improvement.

RAID works by **distributing data across multiple drives** in one of several ways, depending on the RAID level. The most common methods are:

1. **Striping**: Splitting data across multiple drives
2. **Mirroring**: Duplicating data on separate drives
3. **Parity**: Storing calculated recovery information

Key RAID levels:

- RAID 0 (Striping):
    - Data is split across drives
    - Improves performance, no redundancy
    - Example: A 100MB file on 2 drives might use 50MB on each
- RAID 1 (Mirroring):
    - Data is duplicated on separate drives
    - Provides redundancy, slightly improves read performance
    - Example: A 100MB file is written identically to 2 drives
- RAID 5 (Striping with distributed parity):
    - Data and parity are distributed across all drives
    - Good balance of performance and redundancy
    - Example: In a 4-drive array, 3 parts data + 1 part parity per stripe, rotating parity location
- RAID 6 (Striping with double distributed parity):
    - Similar to RAID 5, but with two parity calculations
    - Higher redundancy, can survive two drive failures

Example: Imagine RAID 5 as a team of 4 writers working on a book. Three write content, while one summarizes (parity). If one writer is lost, their work can be reconstructed from the summary and other writers' work.

