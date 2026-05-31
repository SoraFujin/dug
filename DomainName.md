Valid characters: letters, digits, hyphens. Nothing else in a label.

Labels are the parts separated by dots. Each one:
    - Cannot start or end with a hyphen
    - Cannot be empty (so google..com is invalid)
    - Max 63 characters

Total domain: max 253 characters.

For google alone: DNS will try to resolve it as-is. It'll probably fail or return nothing useful, but it's not your job to reject it — let the DNS server deal with it.

What to reject:
    - Empty input
    - Anything with spaces
    - Invalid characters (google!.com, goo gle.com)
    - Empty labels (google..com, .google.com)
    - Labels starting or ending with a hyphen (-google.com)
