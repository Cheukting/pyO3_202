from py202 import order
import threading

burger_pool = []
soda_pool = []

if __name__ == "__main__":
    t1 = threading.Thread(
        target=order, args=("burger soda burger soda soda", burger_pool, soda_pool)
    )
    t2 = threading.Thread(
        target=order, args=("burger soda burger", burger_pool, soda_pool)
    )

    t1.start()
    t2.start()

    t1.join()
    t2.join()

    print("Done!")
