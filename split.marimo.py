import marimo

__generated_with = "0.3.2"
app = marimo.App()


@app.cell
def __():
    prev_water_photo = 797.0
    return prev_water_photo,


@app.cell
def __():
    this_water_photo = 811.0
    return this_water_photo,


@app.cell
def __():
    hawater_consumed = 35.0
    paid = 550850
    return hawater_consumed, paid


@app.cell
def __(left_water):
    left_elec_sha30462 = 1471632

    left_total = left_water + left_elec_sha30462
    return left_elec_sha30462, left_total


@app.cell
def __(back_water):
    back_elec_sha30461 = 1556709

    back_total = back_water + back_elec_sha30461
    return back_elec_sha30461, back_total


@app.cell
def __(hawater_consumed, paid, prev_water_photo, this_water_photo):
    left_consumed = this_water_photo - prev_water_photo
    print(left_consumed)
    left_rate = left_consumed / hawater_consumed
    left_water = left_rate * paid
    print(left_water)
    return left_consumed, left_rate, left_water


@app.cell
def __(hawater_consumed, left_consumed, paid):
    back_consumed = hawater_consumed - left_consumed
    print(back_consumed)
    back_rate = back_consumed / hawater_consumed
    back_water = back_rate * paid
    print(back_water)
    return back_consumed, back_rate, back_water


@app.cell
def __(left_total):
    print(left_total)
    return


@app.cell
def __(back_total):
    print(back_total)
    return


if __name__ == "__main__":
    app.run()
