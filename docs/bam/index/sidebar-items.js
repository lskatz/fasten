initSidebarItems({"constant":[["MAX_BIN","Maximal possible bin value."]],"fn":[["bin_to_region","Returns a maximal region for a given bin."],["region_to_bin","Returns a BAI bin for the record with alignment `[beg-end)`."],["region_to_bins","Returns all possible BAI bins for the region `[beg-end)`."]],"struct":[["Bin","Single bin that stores chunks within the BAM file."],["BinsIter","Iterator over bins."],["Chunk","Chunk `[start-end)`, where `start` and `end` are virtual offsets."],["Index","BAI Index. Allows to get chunks in a bgzip file, that contain records from a specific genomic region."],["LinearIndex","Stores linear index: for each tiling 16384bp window it stores the smallest file offset of an alignment that overlaps it."],["Reference","Index for a single reference sequence. Contains bins and a linear index."],["VirtualOffset","Virtual offset. Represents `block_offset << 16 | contents_offset`, where `block_offset` is `u48` and represents the offset in the bgzip file to the beginning of the block (also known as `coffset` or `compressed_offset`)."]]});