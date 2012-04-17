from __future__ import print_function
from codecs import open # open files as utf-8 on windows
import sys
import xml.etree.cElementTree

# python 2.6 doesn't support multiple contexts
with open('jbo-eng_jbovlaste.dsl', mode='w', encoding='utf-8') as sys.stdout:
    with sys.stdin as f:
        dictionary = xml.etree.cElementTree.parse(f)

    print('#NAME "jbovlaste Lojban<->{0}"\n'.format(
                                     dictionary.find('direction').get('to')))
                                     # etree 1.2 doesn't support [@]

    for v in dictionary.findall('direction/valsi'):
        print(u"""
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
        print(u"""
{word}
    {sense}
    <<{valsi}>>{place}""".format(word=nlw.get('word')
                                ,sense=nlw.get('sense', '')
                                ,valsi=nlw.get('valsi')
                                ,place=nlw.get('place', '')))
