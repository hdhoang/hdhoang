#!/bin/env -S uv tool run --with marimo -- marimo edit

import marimo

__generated_with = "0.14.10"
app = marimo.App()


@app.cell
def _():
    prev_water_photo = 970
    return (prev_water_photo,)


@app.cell
def _(prev_water_photo):
    this_water_photo = 970
    left_consumed = this_water_photo - prev_water_photo
    return (left_consumed,)


@app.cell
def _():
    hawater_consumed = 17
    return (hawater_consumed,)


@app.cell
def _(hawater_consumed, left_consumed):
    back_consumed = hawater_consumed - left_consumed
    return (back_consumed,)


@app.cell
def _(back_water):
    back_elec_sha30461 = 14_999

    back_total = back_water + back_elec_sha30461
    return (back_total,)


@app.cell
def _(back_consumed, back_total, back_water):
    print(back_consumed)

    print(back_water)
    print(int(back_total / 1000))
    return


@app.cell
def _(left_water):
    left_elec_sha30462 = 2_569_234

    left_total = left_water + left_elec_sha30462
    return (left_total,)


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
    left_by_meter = 10 * PRICE_00_TO_10 + max(0, left_consumed - 10.0) * PRICE_10_TO_20
    left_water = left_by_meter * FINAL_FEES_RATIO
    return (left_water,)


@app.cell
def _(FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, back_consumed):
    back_by_meter = back_consumed * PRICE_00_TO_10 + 0 * PRICE_10_TO_20 # + 0 * PRICE_20_TO_30
    back_water = back_by_meter * FINAL_FEES_RATIO

    return (back_water,)


@app.cell
def _():
    # water price ladder
    FINAL_FEES_RATIO = 1.15
    PRICE_00_TO_10 = 8500
    PRICE_10_TO_20 = 9900
    PRICE_20_TO_30 = 16000
    return FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20


if __name__ == "__main__":
    app.run()
