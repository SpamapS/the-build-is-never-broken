class Bus(object):
    def __init__(self, name):
       self.name = name 

    def __str__(self):
        return "%s bus" % (self.name,)

class Train(object):
    def __init__(self, line):
       self.line = line 

    def __str__(self):
        return "%s train" % (self.line,)

class Plane(object):
    def __init__(self, callsign):
       self.callsign = callsign 

    def __str__(self):
        return "%s plane" % (self.callsign,)

b1 = Bus("LAX Flyaway")
t1 = Train("Pacific Coastliner")
p1 = Plane("El Mariachi")

print("I took the {} to the {} and then hopped on the {}.".format(b1, t1, p1))
