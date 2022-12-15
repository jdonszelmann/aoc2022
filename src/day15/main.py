

class Ranges:
    ranges: [(int, int)]

    def __init__(self):
        self.ranges = set()

    @staticmethod
    def overlap(a, b):
        return b[0] <= a[0] <= b[1] or a[0] <= b[0] <= a[1]

    @staticmethod
    def merge_range(a, b):
        return min(a[0], b[0]), max(a[1], b[1])

    def add(self, start, end):
        curr = (start, end)
        # print(f"add {curr}")

        while True:
            for i in self.ranges:
                if self.overlap(i, curr):
                    self.ranges.remove(i)
                    curr = self.merge_range(i, curr)
                    break
            else:
                break

        self.ranges.add(curr)
        # print(self.ranges)

    def __len__(self):
        res = 0
        for start, end in self.ranges:
            res += end - start

        return res

    def __contains__(self, item):
        for i in self.ranges:
            if self.overlap(i, (item, item)):
                return True
        return False


def main():
    inp = [i for i in open("data.test").read().split("\n")]

    def sanitize(a: str):
        return int(a.replace("x=", "").replace("y=", "").replace(",","").replace(":", "").strip())

    # critical_y = 10
    max_xy = 20
    critical_y = 2000000
    # max_xy = 4000000

    def distance(a, b):
        return abs(a[0] - b[0]) + abs(a[1] - b[1])

    sensors = []
    beacons = []

    for i in inp:
        _, _, x, y, _, _, _, _, bx, by = i.split(" ")
        sensor = sanitize(x), sanitize(y)
        beacon = sanitize(bx), sanitize(by)

        sensors.append(sensor)
        beacons.append(beacon)

    for curr_y in range(max_xy):
        points = Ranges()
        for sensor, beacon in zip(sensors, beacons):
            dist = distance(sensor, beacon)
            dist_to_critical = distance(sensor, (sensor[0], curr_y))

            if dist_to_critical <= dist:
                dist_crit_beacon = dist - dist_to_critical
                points.add(max(0, sensor[0] - dist_crit_beacon), min(sensor[0] + dist_crit_beacon, max_xy))
            print(points.ranges)
        print(f"{curr_y} {len(points)}")

        if len(points) != max_xy:
            print(len(points))
            for x in range(max_xy):
                if x not in points:
                    print(f"x={x}, y={curr_y}, {x * 4000000 + curr_y}")
                    return

        print(f"{curr_y}/{max_xy}")


if __name__ == '__main__':
    main()  

# 3698840
