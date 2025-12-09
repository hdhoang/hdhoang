#!/bin/env -S uv tool run --with ruff --with marimo -- marimo edit

import marimo

__generated_with = "0.18.3"
app = marimo.App(width="columns")


@app.cell
def _():
    prev_water_photo = 1029
    return (prev_water_photo,)


@app.cell
def _(prev_water_photo):
    this_water_photo = 1041
    left_consumed = this_water_photo - prev_water_photo

    if left_consumed < 0:
        raise ValueError
    return (left_consumed,)


@app.cell
def _():
    hawater_consumed = 0
    return (hawater_consumed,)


@app.cell
def _(hawater_consumed, left_consumed):
    back_consumed = hawater_consumed - left_consumed

    if back_consumed < 0:
        raise ValueError
    return (back_consumed,)


@app.cell
def _(back_water):
    back_elec_sha30461 = 1205_302

    back_total = back_water + back_elec_sha30461
    return (back_total,)


@app.cell
def _(back_consumed, back_total, back_water):
    print(back_consumed)

    print(back_water)
    print(f"back total: {int(back_total / 1000)}")
    return


@app.cell
def _(left_water):
    left_elec_sha30462 = 1138_752

    left_total = left_water + left_elec_sha30462
    return (left_total,)


@app.cell
def _(left_total):
    print(f"left total: {int(left_total / 1000)}")
    return


@app.cell
def _(left_consumed, left_water):
    print(left_consumed)

    print(left_water)
    return


@app.cell
def _(
    FINAL_FEES_RATIO,
    PRICE_00_TO_10,
    PRICE_10_TO_20,
    PRICE_20_TO_30,
    left_consumed,
):
    left_by_meter = (
        min(10, left_consumed) * PRICE_00_TO_10
        + max(0, left_consumed - 10.0) * PRICE_10_TO_20
        + 0 * PRICE_20_TO_30
    )
    left_water = left_by_meter * FINAL_FEES_RATIO
    return (left_water,)


@app.cell
def _(
    FINAL_FEES_RATIO,
    PRICE_00_TO_10,
    PRICE_10_TO_20,
    PRICE_20_TO_30,
    back_consumed,
):
    back_by_meter = (
        min(10, back_consumed) * PRICE_00_TO_10
        + max(0, back_consumed - 10.0) * PRICE_10_TO_20
    ) + 0 * PRICE_20_TO_30
    back_water = back_by_meter * FINAL_FEES_RATIO
    return (back_water,)


@app.cell
def _():
    # water price ladder
    FINAL_FEES_RATIO = 1.15
    PRICE_00_TO_10 = 8_500
    PRICE_10_TO_20 = 9_900
    PRICE_20_TO_30 = 16_000
    return FINAL_FEES_RATIO, PRICE_00_TO_10, PRICE_10_TO_20, PRICE_20_TO_30


if __name__ == "__main__":
    app.run()
