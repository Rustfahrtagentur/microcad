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

    class Mesh
    {
    public:
        Mesh();
        Mesh(::manifold::Mesh &&mesh);
        ~Mesh();

        std::unique_ptr<::manifold::Mesh> mesh;
    };

    struct Material
    {
        float roughness = 0.2;
        float metalness = 1;
    };

    struct ExportOptions
    {
        bool faceted = true;
        Material mat = {};
    };

    std::unique_ptr<Manifold> sphere(double radius);
    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size);

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold);

    void export_mesh(const std::string &filename, const Mesh &mesh);
}
