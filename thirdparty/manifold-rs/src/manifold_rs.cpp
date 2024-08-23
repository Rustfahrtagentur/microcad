#include "manifold_rs.h"

#include "manifold.h"
// #include "meshIO.h"

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

    Mesh::Mesh() : mesh(std::make_unique<::manifold::Mesh>()) {}

    Mesh::Mesh(::manifold::Mesh &&mesh) : mesh(std::make_unique<::manifold::Mesh>(std::move(mesh))) {}

    Mesh::~Mesh() {}

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold)
    {
        return std::make_unique<Mesh>(manifold.manifold->GetMesh());
    }

    void export_mesh(const std::string &filename, const Mesh &mesh)
    {

        //        Mesh ::manifold::Mesh::Export(filename, *mesh.mesh);
    }

} // namespace manifold_rs
