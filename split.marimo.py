import marimo

__generated_with = "0.6.19"
app = marimo.App()


@app.cell
def __():
    prev_water_photo = 811.0
    return prev_water_photo,


@app.cell
def __():
    this_water_photo = 824.0
    return this_water_photo,


@app.cell
def __():
    hawater_consumed = 39.0
    paid = 675050
    return hawater_consumed, paid


@app.cell
def __(back_water):
    back_elec_sha30461 = 1577127

    back_total = back_water + back_elec_sha30461
    return back_elec_sha30461, back_total


@app.cell
def __(back_total):
    print(back_total)
    return


@app.cell
def __(left_water):
    left_elec_sha30462 = 1641786

    left_total = left_water + left_elec_sha30462
    return left_elec_sha30462, left_total


@app.cell
def __(left_total):
    print(left_total)
    return


@app.cell
def __(left_water, prev_water_photo, this_water_photo):
    left_consumed = this_water_photo - prev_water_photo
    print(left_consumed)

    print(left_water)
    return left_consumed,


@app.cell
def __(FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20):
    left_by_meter = 10 * PRICE_00_TO_10 + 3*PRICE_10_TO_20
    left_water = left_by_meter*FINAL_FEES_RATIO
    return left_by_meter, left_water


@app.cell
def __(back_water, hawater_consumed, left_consumed):
    back_consumed = hawater_consumed - left_consumed
    print(back_consumed)

    print(back_water)
    return back_consumed,


@app.cell
def __(FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, PRICE_20_TO_30):
    back_by_meter = 10 * PRICE_00_TO_10 + 10*PRICE_10_TO_20 + 6*PRICE_20_TO_30
    back_water = back_by_meter*FINAL_FEES_RATIO
    return back_by_meter, back_water


@app.cell
def __():
    # water price ladder
    FINAL_FEES_RATIO = 1.15
    PRICE_00_TO_10 = 8500
    PRICE_10_TO_20 = 9900
    PRICE_20_TO_30 = 16000
    return FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, PRICE_20_TO_30


if __name__ == "__main__":
    app.run()
