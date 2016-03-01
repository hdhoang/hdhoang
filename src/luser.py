from urllib import request, parse

from irc import bot
NAME = 'luser'
lusers = []
luser = bot.SingleServerIRCBot([("chat.freenode.net", 8000)], NAME+'-', NAME)

def change_nick(c, e):
    c.nick(c.get_nickname() + '0')
luser.on_nicknameinuse = change_nick

def join_channels(c, e):
    c.join("#luser-test")
    c.join("#vnluser")
luser.on_welcome = join_channels

def list_lusers(c, e):
    for luser in filter(lambda n: n.startswith(NAME), e.arguments[-1].split(' ')):
        if luser not in lusers:
            lusers.append(luser)
    if c.get_nickname() not in lusers:
        c.reconnect()
    lusers.sort()
luser.on_namreply = list_lusers
def luser_joins(c, e):
    if e.source.nick not in lusers:
        lusers.append(e.source.nick)
        lusers.sort()
luser.on_join = lambda c, e: e.source.startswith(NAME) and luser_joins(c, e)
def luser_quits(c, e):
    lusers.remove(e.source.nick)
luser.on_quit = lambda c, e: e.source.startswith(NAME) and luser_quits(c, e)

def on_pubmsg(c, e):
    msg = e.arguments[0]
    if msg == "report!":
        return c.privmsg(e.target, "operated by hdhoang with source code ???")
    if lusers[len(e.source) % len(lusers)] == c.get_nickname():
        if msg[1:3] == 'g ':
            return c.privmsg(e.target, google(msg[3:]))
        if msg[1:4] == 'wa ':
            return c.privmsg(e.target, wolframalpha(msg[4:]))
        if msg[1:4] == 'tr ':
            return c.privmsg(e.target, translate(msg[4:]))
        if 'http' in msg:
            return
            return c.privmsg(e.target, title(msg))
luser.on_pubmsg = on_pubmsg

def google(text):
    import json
    with request.urlopen('https://ajax.googleapis.com/ajax/services/search/web?v=1.0&rsz=1&q=' + parse.quote(text)) as r:
        data = json.loads(r.read().decode())['responseData']
        if not data['results']:
            return '0 result'
        return data['results'][0]['titleNoFormatting'] + ' ' + data['results'][0]['unescapedUrl']

wolframalpha_key = ''
with open('wolframalpha_key') as f:
    wolframalpha_key = f.read()
def wolframalpha(text):
    import xml.etree.ElementTree as ET
    with request.urlopen('http://api.wolframalpha.com/v2/query?format=plaintext&appid={}&input={}'.format(wolframalpha_key, parse.quote(text))) as r:
        return ' / '.join(n.text for n in ET.parse(r).iter() if n.text and len(n.text.strip())).replace('\n', '')

def title(text):
    return "title not implemented"
def translate(text):
    return "tr not implemented"

luser.start()
