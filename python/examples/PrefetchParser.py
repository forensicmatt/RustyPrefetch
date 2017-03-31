#!/usr/bin/python
import os
import argparse
import json
import logging
import pyrpf

def GetArguments():
    usage = '''Parse Prefetch using pyrpf'''

    arguments = argparse.ArgumentParser(
        description=(usage)
    )
    arguments.add_argument(
        '-s', '--source',
        dest='source',
        action="store",
        required=True,
        type=unicode,
        help=u'The source. File or Directory.'
    )

    return arguments

def GetFilelist(source):
    '''Get a filelist based off of the source of files
    to be parsed.

    :param
        source: The source name (a file or a directory)
    :returns
        filelist: List of files to parse
    '''
    filelist = []
    if os.path.isfile(source):
        filelist.append(source)
    elif os.path.isdir(source):
        for root, dirs, files in os.walk(source):
            for file in files:
                if file.lower().endswith('.pf'):
                    filelist.append(
                        os.path.join(
                            root,
                            file
                        )
                    )
    return filelist

def Main():
    # get arguments to parse
    arguments = GetArguments()
    options = arguments.parse_args()

    COMMIT_COUNT = 10

    filelist = GetFilelist(options.source)
    for filename in filelist:
        with open(filename,"rb") as fh:
            try:
                json_doc = pyrpf.as_json(
                    filename,
                    fh
                )
            except Exception as error:
                logging.error("Error Parsing {}: {}".format(filename,unicode(error)))
                continue

            try:
                print(json_doc)
            except Exception as error:
                logging.error("Error Printing {}: {}".format(filename, unicode(error)))
                continue


if __name__ == "__main__":
    Main()
