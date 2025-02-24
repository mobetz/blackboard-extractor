# Blackboard Assignment Extractor

A small utility to extract files from Blackboard Ultra's "Download All Assignments" .zip archive and sort the contained files into individual folders for each student submission.

![An image showing the location of the Download All button on the assignment grading page](/doc/img/download_all.png)

## About

This tool is written to simplify batch grading of large multi-file project uploads. Blackboard has two options for working with these types of assignments -- navigate to each student and individually download files in their project, or download a single .zip archive containing all 
student submissions in a single folder with mangled names. This tool takes that large .zip and sorts them into submission folders, then attempts to rename them to their original names.

## Usage

1. Download the correct release for your architecture, and open the blackboard-extractor binary to launch a small window.
2. Use the "Browse" buttons to select a source .zip previously from Blackboard Ultra, and a destination folder where the submissions should be extracted.
3. Press the "Extract Files" button to expand the archive into individual submissions.

A CLI interface is also provided, whose arguments can be listed by running `blackboard-extractor-cli --help` from a terminal.