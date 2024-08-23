#include "manifold_rs.h"

#include "manifold.h"

namespace manifold_rs
{
    Manifold::Manifold() : manifold(std::make_unique<::manifold::Manifold>()) {}
    Manifold::Manifold(::manifold::Manifold &&manifold) : manifold(std::make_unique<::manifold::Manifold>(std::move(manifold))) {}
    Manifold::~Manifold() {}

    std::unique_ptr<Manifold> sphere(double radius)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Sphere(radius));
    }

    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cube({x_size, y_size, z_size}));
    }
} // namespace manifold_rs
