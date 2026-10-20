# RAID

# RAID – Redundant Array of Independent Disks

RAID combines multiple physical drives into one logical storage system to improve performance, availability, or both.

**Main goals:**
- **Redundancy:** Continue operating after certain drive failures.
- **Performance:** Increase read/write throughput through parallel I/O.
- **Capacity:** Combine storage from multiple drives.
- **Availability:** Reduce downtime after a drive failure.

**Important:** RAID is not a backup. It does not reliably protect against ransomware, accidental deletion, fire, or controller failure affecting the entire array.

## Core concepts

- **Striping:** Data is split across multiple drives. Improves performance but provides no redundancy by itself.
- **Mirroring:** Identical data is written to multiple drives. Provides redundancy but consumes additional capacity.
- **Parity:** Additional information is calculated from data blocks so missing data can be reconstructed after a drive failure.
- **Hot spare:** An unused drive that can automatically replace a failed drive during rebuilding.
- **Rebuild:** Reconstructing redundant data onto a replacement drive.
- **Controller:** Hardware or software that manages the RAID array.

## RAID capacity calculations

Let:

- \(N\) = number of drives
- \(S\) = capacity of each drive
- \(C\) = usable capacity before filesystem overhead

Assume all drives have the same capacity.

| RAID | Minimum drives | Usable capacity | Drive-failure tolerance |
|---|---:|---|---|
| RAID 0 | 2 | \(N \times S\) | 0 |
| RAID 1 | 2 | \(S\) for a two-drive mirror | 1 in a two-drive mirror |
| RAID 5 | 3 | \((N-1)\times S\) | 1 |
| RAID 6 | 4 | \((N-2)\times S\) | 2 |
| RAID 10 | 4 | \((N/2)\times S\) | At least 1, depending on which drives fail |

Capacity is usually advertised in decimal units: 1 TB = 1,000 GB. Operating systems may display capacity using binary units, so reported values can differ.

## RAID 0 – Striping

**Minimum:** 2 drives  
**Redundancy:** None

Data is distributed across all drives. Multiple drives can process I/O in parallel.

Example with two 2 TB drives:

- Raw capacity: \(2 \times 2 = 4\) TB
- Usable capacity: 4 TB
- Tolerated drive failures: 0

**Advantages**
- High sequential throughput.
- Full combined capacity.
- No parity calculation overhead.

**Disadvantages**
- Failure of any drive can destroy the entire array.
- No redundancy or fault tolerance.

**Use case:** Temporary scratch data or workloads where performance matters and the data can be recreated.

**IHK point:** RAID 0 improves performance and capacity, not availability.

## RAID 1 – Mirroring

**Minimum:** 2 drives  
**Redundancy:** Mirrored copies

The same data is written to both drives.

Example with two 2 TB drives:

- Raw capacity: 4 TB
- Usable capacity: 2 TB
- Tolerated drive failures: 1

**Advantages**
- Simple recovery from a single-drive failure.
- Read performance can improve depending on implementation.
- Straightforward configuration.

**Disadvantages**
- Approximately 50% usable capacity with two equal drives.
- Writes must maintain both copies.

**Use case:** Operating-system drives, small servers, and systems that need simple drive redundancy.

**IHK point:** RAID 1 trades usable capacity for redundancy.

## RAID 5 – Striping with distributed parity

**Minimum:** 3 drives  
**Redundancy:** One drive's worth of distributed parity

Data and parity blocks are distributed across all drives. No single drive is permanently dedicated to parity.

Example with four 2 TB drives:

- Raw capacity: \(4 \times 2 = 8\) TB
- Usable capacity: \((4-1)\times 2 = 6\) TB
- Tolerated drive failures: 1

**Advantages**
- Better capacity efficiency than mirroring.
- Good read performance for many workloads.
- Can reconstruct data after one drive fails.

**Disadvantages**
- Parity calculations and small-write operations add overhead.
- Rebuilds can take a long time on large drives.
- A second drive failure during a degraded period can destroy the array.

**Use case:** Read-heavy file storage where capacity efficiency matters and the risk profile is acceptable.

**IHK point:** RAID 5 can tolerate one failed drive, not two.

## RAID 6 – Striping with double parity

**Minimum:** 4 drives  
**Redundancy:** Two independent parity blocks

Example with six 4 TB drives:

- Raw capacity: \(6 \times 4 = 24\) TB
- Usable capacity: \((6-2)\times 4 = 16\) TB
- Tolerated drive failures: 2

**Advantages**
- Continues operating after any two drive failures.
- Greater protection during rebuilds than RAID 5.
- Suitable for larger storage arrays.

**Disadvantages**
- More capacity used for parity.
- Writes have additional parity overhead.
- Rebuilds can still be lengthy and stressful for remaining drives.

**Use case:** File servers and larger storage systems where tolerating two drive failures is important.

**IHK point:** RAID 6 sacrifices more capacity than RAID 5 for additional fault tolerance.

## RAID 10 (1+0) – Mirroring plus striping

**Minimum:** 4 drives, usually an even number.

RAID 10 creates mirrored pairs and stripes data across those pairs.

Example with four 2 TB drives:

- Raw capacity: \(4 \times 2 = 8\) TB
- Usable capacity: \((4/2)\times 2 = 4\) TB
- Tolerated drive failures: At least one; potentially more if failures occur in different mirrored pairs.

**Advantages**
- High read/write performance.
- No parity calculation required.
- Often faster and more predictable rebuilds than parity RAID.

**Disadvantages**
- Only around 50% of raw capacity is usable.
- More expensive per usable TB.
- Two failed drives in the same mirror pair can destroy the array.

**Use case:** High-performance databases, virtualization storage, and transaction-heavy applications.

**IHK point:** RAID 10 combines the performance of striping with the redundancy of mirroring.

## RAID 50 and RAID 60

These combine multiple parity-based RAID groups using striping across the groups.

### RAID 50 (5+0)

Requires at least six drives: two RAID 5 groups with three drives each.

Example with six 4 TB drives:

- Raw capacity: \(6 \times 4 = 24\) TB
- Usable capacity: \(6-2=4\) drives' worth of capacity, or 16 TB
- Failure tolerance: At least one drive, provided no RAID 5 group loses more than one drive.

**Advantages**
- Higher aggregate performance than a single RAID 5 group in many workloads.
- Better fault isolation between groups.

**Disadvantages**
- Two failures in the same RAID 5 group can destroy the array.
- More complex management.

### RAID 60 (6+0)

Requires at least eight drives: two RAID 6 groups with four drives each.

Example with eight 4 TB drives:

- Raw capacity: \(8 \times 4 = 32\) TB
- Usable capacity: \(8-4=4\) drives' worth of capacity, or 16 TB
- Failure tolerance: Up to two failed drives per RAID 6 group.

**Advantages**
- Can tolerate multiple failures when distributed across groups.
- Suitable for large storage arrays.

**Disadvantages**
- Lower capacity efficiency than RAID 50 with the same number of drives.
- Additional parity overhead and complexity.

**Important:** RAID 50 and RAID 60 are not equally supported by every controller or software implementation.

## Quick comparison

| Level | Usable capacity | Fault tolerance | Typical strength |
|---|---|---|---|
| RAID 0 | 100% | None | Performance and capacity |
| RAID 1 | 50% with two drives | One drive in a two-drive mirror | Simplicity |
| RAID 5 | \(N-1\) drives' worth | One drive per group | Capacity efficiency |
| RAID 6 | \(N-2\) drives' worth | Two drives per group | Higher redundancy |
| RAID 10 | 50% | Depends on failed mirror pairs | Performance and redundancy |
| RAID 50 | Sum of RAID 5 groups' usable capacity | One drive per group | Scaled RAID 5 |
| RAID 60 | Sum of RAID 6 groups' usable capacity | Two drives per group | Scaled RAID 6 |

## Worked IHK calculations

### Example A: RAID 5 capacity

A server has five 3 TB drives configured as RAID 5.

\[
C=(N-1)\times S
\]

\[
C=(5-1)\times3=12\text{ TB}
\]

**Answer:** 12 TB usable capacity before formatting and filesystem overhead.

### Example B: Required drives

A company needs at least 20 TB usable capacity with RAID 6. Each drive has 4 TB.

\[
(N-2)\times4\geq20
\]

\[
N-2\geq5
\]

\[
N\geq7
\]

**Answer:** At least seven drives, ignoring hot spares and implementation-specific constraints.

### Example C: RAID 10 capacity

A server has eight 1.92 TB SSDs in RAID 10.

\[
C=\frac{N}{2}\times S
\]

\[
C=\frac{8}{2}\times1.92=7.68\text{ TB}
\]

**Answer:** 7.68 TB usable capacity before formatting and overhead.

### Example D: RAID 50 fault tolerance

A RAID 50 array has three RAID 5 groups.

- One drive fails in group A.
- One drive fails in group B.
- One drive fails in group C.

The array can continue operating because each group has lost only one drive.

If a second drive fails in group A before recovery, group A loses its RAID 5 redundancy and the entire RAID 50 array may become unavailable or lose data.

## Rebuilds, hot spares and degraded mode

**Degraded mode:** A drive has failed, but the array continues operating using remaining data and redundancy.

**Rebuild:** The array reconstructs missing data onto a replacement drive. Performance may decline during rebuilding.

**Hot spare:** A reserved drive can automatically become the replacement target after a failure.

**Why rebuilds are risky:** Large drives take time to reconstruct. Remaining drives are under load, and another failure or an unrecoverable read error can prevent recovery, depending on RAID level and implementation.

A hot spare reduces the time before rebuilding begins, but does not add another layer of parity to the active array.

## Hardware RAID vs. Software RAID

**Hardware RAID**
- Managed by a dedicated controller.
- May provide controller-level cache and battery/flash-backed protection.
- Depends on controller compatibility and recovery procedures.

**Software RAID**
- Managed by the operating system, such as Linux `mdadm` or Windows Storage Spaces.
- Uses host CPU and memory.
- Can reduce dependence on a specific RAID controller.

Neither is automatically safer or faster in every workload. Configuration, controller quality, cache protection and workload matter.

## RAID vs. backup vs. replication

| Technology | Primary purpose | Main limitation |
|---|---|---|
| RAID | Drive-failure tolerance and/or performance | Does not protect against logical deletion or ransomware |
| Backup | Recover deleted, corrupted or lost data | Recovery depends on backup quality and retention |
| Replication | Maintain copies across systems or locations | Corruption and deletion may replicate |
| High availability | Reduce service interruption | Does not replace data recovery |

**DSGVO connection:** For personal data, appropriate technical and organisational measures should protect availability and resilience. RAID alone is not a complete data-protection strategy.

## Choosing a RAID level

- **RAID 0:** Temporary data where loss is acceptable.
- **RAID 1:** Simple redundancy for a small system.
- **RAID 5:** Capacity-efficient storage with tolerance for one drive failure.
- **RAID 6:** Larger arrays requiring tolerance for two drive failures.
- **RAID 10:** High-performance workloads needing redundancy.
- **RAID 50/60:** Larger arrays that benefit from multiple RAID groups.

Choose based on usable capacity, write workload, required fault tolerance, rebuild risk, cost and recovery objectives.

## 3-line IHK answer

RAID combines multiple drives to improve performance, availability or fault tolerance through striping, mirroring and/or parity.  
The usable capacity and number of tolerated drive failures depend on the RAID level and group layout.  
RAID is not a backup; independent backups and tested recovery procedures are still required.

## Siehe auch

- [Snapshots-Backups](snapshots-backups.md) – RAID ersetzt kein Backup
- [Schutzziele](schutzziele.md) – Verfügbarkeit durch Redundanz
- [CPU](cpu.md) – Controller, I/O und Performance

