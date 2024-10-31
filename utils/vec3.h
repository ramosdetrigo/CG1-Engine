#ifndef VEC3_H
#define VEC3_H

#include <cmath>
#include <stdexcept>
using namespace std;

float clip(float n, float lower, float upper) {
  return std::max(lower, std::min(n, upper));
}

struct Vec3 {
    float x, y, z;
    Vec3 ():  x(0), y(0), z(0) {};
    Vec3 (float x, float y, float z): x(x), y(y), z(z) {};
    
    inline Vec3 operator+(const Vec3& other) { return Vec3(x+other.x, y+other.y, z+other.z); }

    inline Vec3 operator-(const Vec3& other) { return Vec3(x-other.x, y-other.y, z-other.z); }

    inline Vec3 operator-() const { return Vec3(-x, -y, -z); }

    inline Vec3 operator*(const float other) const { return Vec3(x*other, y*other, z*other); }

    inline friend Vec3 operator*(float scalar, const Vec3& other) { return Vec3(other.x*scalar, other.y*scalar, other.z*scalar); }

    inline Vec3 operator*(const Vec3& other) const { return Vec3(x*other.x, y*other.y, z*other.z); }

    inline Vec3 operator/(const float other) const { return Vec3(x/other, y/other, z/other); }

    inline friend Vec3 operator/(float scalar, const Vec3& other) { return Vec3(other.x/scalar, other.y/scalar, other.z/scalar); }

    inline Vec3 operator/(const Vec3& other) const { return Vec3(x/other.x, y/other.y, z/other.z); }

    inline bool operator==(const Vec3& other) const { return (x == other.x) && (y == other.y) && (z == other.z); }

    inline float dot(const Vec3& other) const { return x*other.x + y*other.y + z*other.z; }

    inline float magnitude() const { return sqrt(x*x + y*y + z*z); }

    inline Vec3 normalize() const { return *this / magnitude(); }

    inline Vec3 rgb_normalized() const { return Vec3(x/255.0, y/255.0, z/255.0); }

    inline Vec3 rgb_255() const { return Vec3(x*255.0, y*255.0, z*255.0); }

    inline Vec3 clamp(float min, float max) { return Vec3(clip(x, min, max), clip(y, min, max), clip(z, min, max)); }

    inline friend std::ostream& operator<<(std::ostream& os, const Vec3& vec) {
        os << "Vec3(" << vec.x << ", " << vec.y << ", " << vec.z << ")";
        return os;
    }
};

#endif