* split this off into its own repo
* take note input from json and output table with notes and intervals
* take interval input from json and output table with notes and intervals
* transpose scale to specified key
* transpose scale to all 12 keys of chromatic scale
* output all possible modes of scale for specified key (or -a for all 12 keys of chromatic scale)
* generate 84 charts of modes for seven-note scales that would cover the entire chromatic scale (this will not work for certain scales, such as the whole-tone scale)

stretch goals:

* `rephrase_with_flats`/`rephrase_with_sharps` method in `scale` for filling in new notes (defaults to false, since most DAWs default to sharps for most of their notation)
* output format
	* currently, we only output to HTML, but we take JSON input. we should add other options to output to JSON, CSV, or possibly a few other common formats
	* for HTML output, make it possible to add custom formatting parameters to notes, intervals, or both, depending either on values, offsets, positions within the scale, patterns, etc.
* automatically find links to scales’ pages on Ian Ring’s site
* potential `no_consecutive_same_letter` method for `scale` (or some better way to handle this; Solra says that flags like this are more trouble than they’re worth, and Aaron has agreed to an armistice on this topic for now :P)
* potential method to automatically fill in last interval of scale whose intervals are between -12 and 12 (non-inclusive)? (this might also be more trouble than it’s worth)
