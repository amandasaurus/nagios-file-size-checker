Command to check that if a specified file exists that it is within a certain
size, exiting with the appropriate nagios status codes

The motivation was a data update process which would create intermediate files
during processing. If the file size gets too large, you have a problem. I
wanted to make a nagios check to detect this.

Usage:

    nagios-file-size-checker FILENAME MIN_SIZE_BYTES MAX_SIZE_BYTES

e.g.

    nagios-file-size-checker /srv/update-data/stagedfile 0 10000


Copyright © 2017 Rory McCann <rory@technomancy.org>, Licenced GNU Affero GPL v3 (or later)
