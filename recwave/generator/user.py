import uuid
from pprint import pprint
import numpy as np
from preference import Preferences
from routines import generate_routine_dict


def new_user():
    id = str(uuid.uuid1())

    activeness = np.exp(np.random.lognormal(mean=1))
    routines = generate_routine_dict(activeness)
    distrib = dict(userid=id, activeness=activeness, routines=routines)

    for tag, val in zip(Preferences.tags, np.exp(
        np.random.lognormal(mean=1, sigma=0.5, size=len(Preferences.tags)))):
        distrib[tag] = val
    return distrib


if __name__ == "__main__":
    pprint(new_user(), indent=2)
