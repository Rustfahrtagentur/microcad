#pragma once

#include "rust/cxx.h"
#include <memory>

namespace manifold
{
    class Manifold;
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
