"""
Example usage of the `oink` callback functins.
"""

from oink import reg_cb, call_cb


def the_callback():
    print("at the_callback()")


# register the callback
reg_cb(the_callback)
# invoke the callback
call_cb()
