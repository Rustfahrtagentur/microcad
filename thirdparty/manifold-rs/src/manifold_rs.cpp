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

    Mesh::Mesh() : mesh(std::make_unique<::manifold::Mesh>()) {}

    Mesh::Mesh(::manifold::Mesh &&mesh) : mesh(std::make_unique<::manifold::Mesh>(std::move(mesh))) {}

    Mesh::~Mesh() {}

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold)
    {
        return std::make_unique<Mesh>(manifold.manifold->GetMesh());
    }

    std::unique_ptr<std::vector<float>> mesh_vertices(const Mesh &mesh)
    {
        std::vector<float> vertices;
        vertices.reserve(mesh.mesh->vertPos.size() * 6);
        assert(mesh.mesh->vertPos.size() == mesh.mesh->vertNormal.size());
        for (size_t i = 0; i < mesh.mesh->vertPos.size(); i++)
        {
            vertices.push_back(mesh.mesh->vertPos[i].x);
            vertices.push_back(mesh.mesh->vertPos[i].y);
            vertices.push_back(mesh.mesh->vertPos[i].z);
            vertices.push_back(mesh.mesh->vertNormal[i].x);
            vertices.push_back(mesh.mesh->vertNormal[i].y);
            vertices.push_back(mesh.mesh->vertNormal[i].z);
        }
        return std::make_unique<std::vector<float>>(vertices);
    }

    std::unique_ptr<std::vector<uint32_t>> mesh_indices(const Mesh &mesh)
    {
        std::vector<uint32_t> indices;
        indices.reserve(mesh.mesh->triVerts.size() * 3);
        for (size_t i = 0; i < mesh.mesh->triVerts.size(); i++)
        {
            indices.push_back(mesh.mesh->triVerts[i].x);
            indices.push_back(mesh.mesh->triVerts[i].y);
            indices.push_back(mesh.mesh->triVerts[i].z);
        }
        return std::make_unique<std::vector<uint32_t>>(indices);
    }
} // namespace manifold_rs
