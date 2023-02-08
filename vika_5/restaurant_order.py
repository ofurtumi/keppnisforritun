import sys


def mul_orders(orders, menu, i, j, cost):
    order1 = trace_order(orders, menu, i, cost)
    order2 = trace_order(orders, menu, j, cost)
    if order1 == order2:
        return False
    return True


def compute_order_list(orders, menu, n):
    none_list = [None for _ in range(max(orders) + 1)]
    none_list[0] = -1

    for cost in range(1, len(none_list)):
        for i in range(n):

            if cost < min(menu):
                none_list[cost] = 'Impossible'
                break

            if cost - menu[i] >= 0:
                order = none_list[cost - menu[i]]

                if type(order) == int:
                    if type(none_list[cost]) == int:
                        if mul_orders(none_list, menu, none_list[cost], order, cost):
                            none_list[cost] = 'Ambiguous'
                            break
                    else:
                        none_list[cost] = i

                elif order == 'Ambiguous':
                    none_list[cost] = order
                    break

                else:
                    if type(none_list[cost]) != int:
                        none_list[cost] = order

    return none_list


def trace_order(order_list, menu, last_index, cost):
    rv = []
    if last_index == -1:
        return [cost]
    while cost > 0:
        if type(last_index) == str:
            return last_index
        rv.append(last_index + 1)
        cost -= menu[last_index]
        last_index = order_list[cost]
    return sorted(rv)


def main(input):
    n = int(input.readline())
    assert 1 <= n <= 100, 'Number of items out of range'

    menu = [int(cost) for cost in input.readline().split()]
    assert len(menu) == n, 'Number of items unmatched'
    assert max(menu) <= 1000, 'No item costs more than 1000 SEK'

    m = int(input.readline())
    assert 1 <= m <= 1000

    orders = [int(order) for order in input.readline().split()]
    assert len(orders) == m, 'Number of orders unmatched'
    assert 1 <= max(orders) <= 30000, 'Total cost of some orders out of range'

    lst = compute_order_list(orders, menu, n)
    for cost in orders:
        order = lst[cost]
        if type(order) == int:
            rv = trace_order(lst, menu, order, cost)
            print(*rv)
        else:
            print(order)


if __name__ == '__main__':
    main(sys.stdin)
