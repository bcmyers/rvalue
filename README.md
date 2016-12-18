# rvalue

The beginning of a script to pull down information on S&P 500 weightings.  The eventual goal is to use this to calculate S&P 500 "dispersion"

monthly_dispersion = sum_over_i(square_root(index_weight_i * (monthly_return_i - montgly_return_S&P500)^2)) where i represents one stock in S&P500
