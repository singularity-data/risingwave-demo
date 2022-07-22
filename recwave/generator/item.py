import uuid
import numpy as np
from preference import Preferences
from pprint import pprint


def new_item():
    id = str(uuid.uuid1())

    # activeness marks how frequent the user interact with merchandises
    popularity = np.exp(np.random.lognormal(mean=1))
    distrib = dict(itemid=id, popularity=popularity)

    for tag, val in zip(Preferences.tags, np.exp(
        np.random.lognormal(mean=1, size=len(Preferences.tags)))):
        distrib[tag] = val

    return distrib


if __name__ == "__main__":
    pprint(new_item())
