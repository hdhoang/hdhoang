#!/bin/env -S uv tool run -- marimo edit
# /// script
# dependencies = ["marimo"]
# /// script

import marimo

__generated_with = "0.11.6"
app = marimo.App()


@app.cell
def _():
    prev_water_photo = 913
    return (prev_water_photo,)


@app.cell
def _(prev_water_photo):
    this_water_photo = 925
    left_consumed = this_water_photo - prev_water_photo
    return left_consumed, this_water_photo


@app.cell
def _():
    hawater_consumed = 13
    return (hawater_consumed,)


@app.cell
def _(back_water):
    back_elec_sha30461 = 174_046

    back_total = back_water + back_elec_sha30461
    return back_elec_sha30461, back_total


@app.cell
def _(back_total):
    print(int(back_total / 1000))
    return


@app.cell
def _(left_water):
    left_elec_sha30462 = 831_049

    left_total = left_water + left_elec_sha30462
    return left_elec_sha30462, left_total


@app.cell
def _(left_total):
    print(int(left_total / 1000))
    return


@app.cell
def _(left_consumed, left_water):
    print(left_consumed)

    print(left_water)
    return


@app.cell
def _(FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, left_consumed):
    left_by_meter = 10 * PRICE_00_TO_10 + (left_consumed - 10.0) * PRICE_10_TO_20
    left_water = left_by_meter * FINAL_FEES_RATIO
    return left_by_meter, left_water


@app.cell
def _(back_water, hawater_consumed, left_consumed):
    back_consumed = hawater_consumed - left_consumed
    print(back_consumed)

    print(back_water)
    return (back_consumed,)


@app.cell
def _(FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, PRICE_20_TO_30):
    back_by_meter = 1* PRICE_00_TO_10 + 0 * PRICE_10_TO_20 + 0 * PRICE_20_TO_30
    back_water = back_by_meter * FINAL_FEES_RATIO
    return back_by_meter, back_water


@app.cell
def _():
    # water price ladder
    FINAL_FEES_RATIO = 1.15
    PRICE_00_TO_10 = 8500
    PRICE_10_TO_20 = 9900
    PRICE_20_TO_30 = 16000
    return FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, PRICE_20_TO_30


if __name__ == "__main__":
    app.run()
