#pragma once

#include "rust/cxx.h"
#include <memory>

namespace manifold
{
    class Manifold;
    struct Mesh;
} // namespace manifold

namespace manifold_rs
{
    class Manifold
    {
    public:
        Manifold();
        Manifold(::manifold::Manifold &&manifold);
        ~Manifold();

        std::unique_ptr<::manifold::Manifold> manifold;
    };

    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments);
    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size);

    std::unique_ptr<Manifold> union_(const Manifold &a, const Manifold &b);
    std::unique_ptr<Manifold> intersection(const Manifold &a, const Manifold &b);
    std::unique_ptr<Manifold> difference(const Manifold &a, const Manifold &b);

    class Mesh
    {
    public:
        Mesh();
        Mesh(::manifold::Mesh &&mesh);
        ~Mesh();

        std::unique_ptr<std::vector<float>> vertices() const;
        std::unique_ptr<std::vector<uint32_t>> indices() const;

        std::unique_ptr<::manifold::Mesh> mesh;
    };

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold);
    std::unique_ptr<Manifold> manifold_from_mesh(const Mesh &mesh);
}
