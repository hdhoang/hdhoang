from http import client

from irc import bot
NAME = 'luser'
lusers = []
luser = bot.SingleServerIRCBot([("chat.freenode.net", 8000)], NAME+'-py', NAME)

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
        c.privmsg(e.target, "operated by hdhoang with source code ???")
    if lusers[len(e.source) % len(lusers)] == c.get_nickname():
        if msg[1:3] == 'g ':
            return c.privmsg(e.target, google(msg[3:]))
        if msg[1:4] == 'wa ':
            c.privmsg(e.target, wolframalpha_key(msg[4:]))
        if msg[1:4] == 'tr ':
            c.privmsg(e.target, translate(msg[4:]))
        if 'http' in msg:
            c.privmsg(e.target, title(msg))
luser.on_pubmsg = on_pubmsg

def google(text):
    import json
    connection = client.HTTPSConnection('ajax.googleapis.com')
    connection.request('GET','/ajax/services/search/web?v=1.0&rsz=1&q=' + text)
    data = json.loads(connection.getresponse().read().decode())['responseData']
    if len(data['results']) == 0:
        return '0 result'
    return data['results'][0]['titleNoFormatting'] + ' ' + data['results'][0]['unescapedUrl']

def wolframalpha(text):
    return "wa not implemented"
def title(text):
    return "title not implemented"
def translate(text):
    return "tr not implemented"

luser.start()
