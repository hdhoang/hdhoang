from __future__ import print_function
from codecs import open
import sys
import xml.etree.cElementTree

def help():
    print("""Usage: {0} xml dsl
    with XML file from http://jbovlaste.lojban.org/export/xml.html.
    - means read from stdin or write to stdout.
    """.format(sys.argv[0]))

if len(sys.argv) != 3:
    help()
    sys.exit(1)

in_file = sys.argv[1] if sys.argv[1] != '-' else sys.stdin
dictionary = xml.etree.cElementTree.parse(in_file)

if sys.argv[-1] != '-':
    out_file = open(sys.argv[-1], mode='w', encoding='utf-8')
else:
    out_file = sys.stdout

out_file.write(u"""#NAME "jbovlaste Lojban<->{0}"
#INDEX_LANGUAGE "Lojban"
#CONTENTS_LANGUAGE  "{0}"
""".format(dictionary.find('direction').get('to')))
                                     # etree 1.2 doesn't support [@]

for v in dictionary.findall('direction/valsi'):
        out_file.write(u"""
{0[word]} {rafsis}
    Type: {0[type]} {selmaho}
    {definition}
    {notes}""".format(v.attrib
                     ,rafsis=' '.join(['-{0.text}-'.format(r)
                                       for r in v.findall('rafsi')])
                     ,selmaho=v.findtext('selmaho', '')
                     ,definition=v.findtext('definition')
                     ,notes=v.findtext('notes', '')
                             .replace('{','<<').replace('}','>>')
                             ))

for nlw in dictionary.findall('direction/nlword'):
        out_file.write(u"""
{word}
    {sense}
    <<{valsi}>>{place}""".format(word=nlw.get('word')
                                ,sense=nlw.get('sense', '')
                                ,valsi=nlw.get('valsi')
                                ,place=nlw.get('place', '')))

out_file.close()
