// Special convolution and threshold function for solving day4
__kernel void conv2d_threshold(
    __global const float* input,
    __global float* output,
    __global int* accessible_count,
    __constant float* k,
    const float activation,
    const int width,
    const int height
) {
    int x = get_global_id(0);
    int y = get_global_id(1);
    if (x >= width || y >= height) return;

    if (input[y * width + x] <= 0.0f) {
        output[y * width + x] = 0.0f;
        return;
    }

    float sum = 0.0f;

    int dx[3] = {-1, 0, 1};
    int dy[3] = {-1, 0, 1};

    for (int i = 0; i < 3; i++) {
        int sy = y + dy[i];
        if (sy < 0 || sy >= height) continue;
        for (int j = 0; j < 3; j++) {
            int sx = x + dx[j];
            if (sx < 0 || sx >= width) continue;
            float neighbor = input[sy * width + sx];
            if (neighbor > 0.0f) sum += neighbor * k[i * 3 + j];
        }
    }

    if (sum >= activation) {
        output[y * width + x] = 1.0f;
    } else {
        output[y * width + x] = -1.0f;
        atomic_inc(accessible_count);
    }
}
