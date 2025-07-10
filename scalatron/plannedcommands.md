* generate 84 charts of modes for seven-note scales that would cover the entire chromatic scale (this will not work for certain scales, such as the whole-tone scale)
* take note input from json and output table with notes and intervals
* take interval input from json and output table with notes and intervals
* transpose scale to specified key
* transpose scale to all 12 keys of chromatic scale
* output all possible modes of scale for specified key (or -a for all 12 keys of chromatic scale)
* user can specify a no_consecutive_same_letter flag

stretch goals:

* output format
	* currently, we only output to HTML, but we take JSON input. it might make sense to add options to also output to JSON, CSV, or possibly a few other common formats
	* for HTML output, make it possible to add custom formatting parameters to notes, intervals, or both, depending either on values, offsets, positions within the scale, patterns, etc.
* allow scales to reuse letters consecutively if pitches aren’t the same (e.g., C, C♯, D♯, E, F, G, G♯, A, B)
* automatically find links to scales’ pages on Ian Ring’s site
