while True:
    x = input("Please guess the password: ")

    if x == "changeme":
        print("Correct!")
        break
    else:
        print("Please try again.")