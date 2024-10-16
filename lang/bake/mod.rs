
trait Bake2D {
    fn bake2d(&self, node: ModelNode) -> Result<geo2d::Geometry>;
}

trait Bake3D {
    fn bake3d(&self, node: ModelNode) -> Result<geo2d::Geometry>;
}

