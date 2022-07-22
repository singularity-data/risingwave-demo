import numpy as np


def generate_routine_dict(activeness):
    """
    Generate a "routine" dict of {view, click, purchase}
    :param activeness:
    :return {view: {delay: ... conversion: ...} ...}
    """
    routine = {}
    for tag in ["view", "click", "purchase"]:
        routine[tag] = {
            "delay": 1 / activeness,
            "conversion": activeness / 100 * np.random.uniform(0.5, 1.5)
        }
    return routine
