#pragma once

#include "rust/cxx.h"
#include <memory>

#ifdef MANIFOLD_RS_STATIC_DEFINE
#define MANIFOLD_RS_EXPORT
#define MANIFOLD_RS_NO_EXPORT
#else
#ifndef MANIFOLD_RS_LIBRARY
/* We are building this library */
#define MANIFOLD_RS_EXPORT __declspec(dllimport)
#else
/* We are using this library */
#define MANIFOLD_RS_EXPORT __declspec(dllexport)
#endif

#ifndef MANIFOLD_RS_NO_EXPORT
#define MANIFOLD_RS_NO_EXPORT
#endif
#endif

namespace manifold
{
    class Manifold
    {
    };
} // namespace manifold

namespace manifold_rs
{
    class Manifold
    {
    public:
        Manifold();
        Manifold(::manifold::Manifold &&manifold);
        ~Manifold();

    private:
        std::unique_ptr<::manifold::Manifold> manifold;
    };

    std::unique_ptr<Manifold> sphere(double radius);
    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size);
}
