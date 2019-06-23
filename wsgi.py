from api.api import application
from flask_restful import Api
from api.api import User, Identify, Interpret, Coggers, ListGuilds, Invite
from multiprocessing import Process, Pipe, Queue
#from asyncio import Queue
import bot2
from bot2 import coolbot
from src.config import secret_token
import zmq

#export PYTHON=python3.6; uwsgi --ini config_uwsgi.ini --http :5061 --wsgi-file wsgi.py
print("wsgi loaded")

q = Queue()
api = Api(application)
api.add_resource(User, "/user/<string:name>", resource_class_kwargs={'q' : q})
api.add_resource(Identify, "/identify")
api.add_resource(ListGuilds, "/guilds")
api.add_resource(Invite, "/invite/<string:guild_id>")
api.add_resource(Interpret, "/interpret", resource_class_kwargs={'q' : q})
api.add_resource(Coggers, "/coggers/<string:extension>", resource_class_kwargs={'q' : q})

coolbot.q = q
p = Process(name="autbot", target=coolbot.run, args=(secret_token,))
p.start()
if __name__ == '__main__':
    application.run()
    #p.join()