import json
import click
import user
import item

@click.command()
@click.option('--num-users', default=200, help='Number of users to be generated')
@click.option('--num-items', default=20, help='Number of items to be generated')
@click.option('--dump-users', default='./users.json', help="The location to dump the `user` json file")
@click.option('--dump-items', default='./items.json', help="The location to dump the `item` json file")
def generate(num_users, num_items, dump_users, dump_items):
    """Simple program that greets NAME for a total of COUNT times."""
    users = [user.new_user() for _ in range(num_users)]
    items = [item.new_item() for _ in range(num_items)]
    json.dump(users, open(dump_users, 'w'), indent=2)
    json.dump(items, open(dump_items, 'w'), indent=2)


if __name__ == '__main__':
    generate()
